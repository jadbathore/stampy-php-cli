<?php

namespace  Stampy\Extension;

use Stampy\Model\Trait\Coloring;
use Composer\Composer;
use Composer\EventDispatcher\EventSubscriberInterface;
use Composer\IO\IOInterface;
use Composer\Plugin\PluginInterface;
use Composer\Script\Event;
use Composer\Console\Application;
use stdClass;
use Symfony\Component\Console\Input\ArrayInput;
use Symfony\Component\Console\Output\BufferedOutput;

class Plugin implements PluginInterface,EventSubscriberInterface {

    use Coloring;

    public function activate(Composer $composer, IOInterface $io){}
    public function deactivate(Composer $composer, IOInterface $io){}
    public function uninstall(Composer $composer, IOInterface $io){}

    public static function getSubscribedEvents()
    {
        return [
            'post-install-cmd' => 'build',
            'post-update-cmd' => 'build',
        ];
    }

    public function build(Event $event){
        $config = $event->getComposer()->getConfig();
        $vendorDir = $config->get('vendor-dir'); 
        $pathComposer = dirname($vendorDir)."/composer.json";
        $json = json_decode(file_get_contents($pathComposer));
        $stampy = $json?->{"stampy"} ?? true;
        if($stampy?->rebuild_after_install_or_update ?? $stampy){ 
            $this->install_update($event,$json,$pathComposer);
        }
    }

    private function install_update(Event $event,mixed $json,string $pathComposer)
    {
        $io = $event->getIO();
        $this->handlePreCompile($io,$exitCode,$pathExt);
        $output = exec("vendor/stampy/php-cli/init/install \"$exitCode\" $pathExt",result_code:$code);
        register_shutdown_function(function() use (&$code,&$io,$json,$pathComposer){
            switch($code){
                case 0;
                    $io->write("\n\t✨welcome to Stampy!✨\n");
                    $this->color("Stampy successfully install","green");
                break;
                case 64:
                    exec("tty",$tty);
                    $tty = implode($tty);
                    $output = shell_exec("./vendor/bin/dockerStampy < $tty > $tty 2>&1");
                    $io->write($output);
                break;
                case 130:
                    $io->writeError(
                        $this->textColor("You prematurely stopped the shell script during the installation of Stampy","bgred")
                    );
                default:
                    $io->writeError(
                        $this->textColor("unable to install stampy due to a installation error exitCode:$code","bgred")
                    );
            }
            if ($code == 0 || $code == 64){
                $executableComposer = new Application();
                $executableComposer->setAutoExit(false);
                $input = new ArrayInput([
                    'command' => 'dump-autoload',
                    '--optimize' => true
                ]);
                $output = new BufferedOutput();
                $executableComposer->run($input,$output);
                $io->write("\n".$this->color($output->fetch(),"green"));
                if(!isset($json?->{"stampy"}?->rebuild_after_install_or_update)){ 
                    $json->stampy = new stdClass();
                    $json->{"stampy"}->rebuild_after_install_or_update = false;
                    if (!isset($json?->{"script"})){
                        $json->script = new stdClass();
                    }
                    $json->script->dockerStampy =  "./vendor/bin/dockerStampy";
                    $json->script->execdockerStampy =  "./vendor/bin/execDockerStampy";
                    $encode = json_encode($json, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES);
                    file_put_contents($pathComposer,$encode);
                }

                
            }
        });
        $io->write($output);
    }

    private function handlePreCompile(\Composer\IO\IOInterface $io,int|null &$exitCode,string|null &$code = ""):void
    {
        exec("vendor/stampy/php-cli/init/preCompileOption",output:$out,result_code:$preCompileCode);
        switch($preCompileCode){
            case 148:
                $input = $io->ask(
                    $this->textColor(
                        "the stampy extension add no pré-compile binairy for your architecture you can compile the binairy by yourself
                        \rusing cargo or use docker.If you using cargo make sure you got cargo install (https://rust-lang.org/tools/install/).
                        \rIf you using docker make sure you docker daemon running","green","bold").$this->textColor(" [cargo|docker] ","yellow","bold").$this->textColor("?","green","bold")
                );
                $clearCount=4;
                $confirm = false;
                while($confirm == false){
                    $conform = ($input == "cargo"||$input == "docker");
                    if($conform == false) {
                        $input = $io->ask(
                            $this->textColor("('$input') is not a valid input you must choose between [cargo/docker] ? ","yellow",'underline')
                        );
                        $clearCount++;
                    } else {
                        $confirm = $conform;
                    }
                }
                $exitCode = ($input == "cargo")?2:1;
                $io->write(str_repeat("\033[A\r\033[2K",$clearCount));
                break;
            case 0:
            case 1:
                $exitCode = $preCompileCode;
                $code = implode($out);
            break;
            default:
            $io->writeError(
                $this->textColor("unable to use the shell the configure the pre-compile binairy exit code :$preCompileCode","bgred")
            );
            exit;
        }
    }


}
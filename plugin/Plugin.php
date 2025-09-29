<?php

namespace  Stampy\Extension;

use Stampy\Model\Trait\Coloring;
use Composer\Composer;
use Composer\EventDispatcher\EventSubscriberInterface;
use Composer\IO\IOInterface;
use Composer\Plugin\PluginInterface;
use Composer\Script\Event;

class Plugin implements PluginInterface,EventSubscriberInterface {

    use Coloring;
    public function activate(Composer $composer, IOInterface $io)
    {
        $io->write("\n\t✨welcome to Stampy!✨\n");
    }

    public function deactivate(Composer $composer, IOInterface $io)
    {
        
    }

    public function uninstall(Composer $composer, IOInterface $io)
    {
        
    }

    public static function getSubscribedEvents()
    {
        return [
            'post-install-cmd' => 'install_update',
            'post-update-cmd' => 'install_update',
        ];
    }

    public function install_update(Event $event)
    {
        $io = $event->getIO();
        // exec("vendor/stampy/php-cli/init/preCompileOption",$precompileOutput,result_code:$preCompileCode);
        // $io->write($precompileOutput);
        $this->handlePreCompile($io,$exitCode,$pathExt);
        $output = exec("vendor/stampy/php-cli/init/install \"$exitCode\" $pathExt",result_code:$code);

        register_shutdown_function(function() use (&$code,&$io){
            switch($code){
                case 1;
                    $this->color("Stampy successfully install","green");
                break;
                case 130:
                    $io->writeError(
                        $this->textColor("You prematurely stopped the shell script during the installation of Stampy","bgred")
                    );
                case 2:
                default:
                    // echo $code;
                    $io->writeError(
                        $this->textColor("unable to install stampy due to a installation error","bgred")
                    );
                    // exit;
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
                        \rusing cargo or use docker.If you using cargo make sure you got cargo install (https://doc.rust-lang.org/cargo/commands/cargo-install.html).
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
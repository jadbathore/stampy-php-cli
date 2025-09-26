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
        $output = exec("vendor/stampy/php-cli/init/install",result_code:$code);

        register_shutdown_function(function() use (&$code,&$io){
            switch($code){
                case 1;
                    $this->color("Stampy successfully install","color");
                break;
                case 130:
                    $io->writeError(
                        $this->textColor("You prematurely stopped the shell script during the installation of Stampy","bgred")
                    );
                case 2:
                default:
                    $io->writeError(
                        $this->textColor("unable to install stampy due to a installation error","bgred")
                    );
                    exit;
            }
        });

        $io->write($output);
    }
}
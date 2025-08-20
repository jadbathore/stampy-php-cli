<?php

namespace  Stampy\Extension;

use Composer\Composer;
use Composer\EventDispatcher\EventSubscriberInterface;
use Composer\IO\IOInterface;
use Composer\Plugin\PluginInterface;
use Composer\Script\Event;

class Plugin implements PluginInterface,EventSubscriberInterface {

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
        $output = shell_exec("vendor/stampy/php-cli/init/install");
        $io->write($output);
    }

}
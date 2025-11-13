<?php

namespace Stampy\Model\Class\SingleTone;

use Stampy\Model\Interface\SingleToneInterface;

use ConsoleTTY;
use const Stampy\STDOUT_KEY;
use const Stampy\STDERR_KEY;


class StampyConsole implements SingleToneInterface {

    private ConsoleTTY $stdout_tty;
    private  ConsoleTTY $stderr_tty;
    private static ?StampyConsole $instance;

    private function __construct(){
        $this->stdout_tty = new ConsoleTTY(STDOUT_KEY);
        $this->stderr_tty = new ConsoleTTY(STDERR_KEY);
    }

    public function __destruct()
    {
        echo PHP_EOL;
    }
    
    private function __clone(){}

    public function __wakeup()
    {
        throw new \Exception("Cannot unserialize a singleton.");
    }

    public function init(mixed ...$args): void{
        self::$instance =  new static(... $args);
    }

    public static function &instance(mixed ...$args):Self
    {   
        {
            if(!isset(self::$instance))
            {
                self::$instance =  new static(... $args);
            }
            return self::$instance;
        }
    }

    public function getStdErrTTY(): ConsoleTTY 
    {
        return $this->stderr_tty;
    }

    public function getStdOutTTY(): ConsoleTTY 
    {
        return $this->stdout_tty;
    }

}
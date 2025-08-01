<?php

namespace Stampy\Model\Class\Singletone;

use Stampy\Model\Class\Object\Method_CLI;
use Stampy\Model\Class\throwable\BinError;
use Stampy\Model\Interface\SingleToneInterface;
use Stampy\Model\Trait\Coloring;
use Stampy\Model\Enum\Error;
use Stampy\Model\Interface\MethodCLIInterface;

class BinErrorHandler implements SingleToneInterface {
    use Coloring;
    private static ?BinErrorHandler $instance;

    private function __construct(
        private ?BinError $binError=null,
    ) {}
    
    private function __clone()
    {}

    public function __destruct()
    {
        echo PHP_EOL;
    }
    public function __wakeup()
    {
        throw new \Exception("Cannot unserialize a singleton.");
    }

    public function init(mixed ...$args): void{
        self::$instance =  new static(... $args);
    }

    public static function &instance(mixed ...$args):BinErrorHandler
    {   
        {
            if(!isset(self::$instance))
            {
                self::$instance =  new static(... $args);
            }
            return self::$instance;
        }
    }

    private function doubleCommandError(MethodCLIInterface $method1,?MethodCLIInterface $method2 = null){
        $this->color("You have double Command Name",'red','bold');
        echo PHP_EOL;
        $info = function(?MethodCLIInterface $method = null){
            $this->color("In file " ,'blue');
            $this->color($method->getFile() ,'bgblue');
            echo PHP_EOL;
            $this->color($method->getClass()."()->".$method->getName().'()','yellow','underline');
            $this->color(" in line ",'yellow');
            $this->color($method->getLine(),'yellow','underline');
            echo PHP_EOL;
        };
        $info($method1);
        $info($method2);
        echo PHP_EOL;
        $this->color("You might want to change one of those two command Attribut:",'green','underline');
        echo PHP_EOL;
        $this->color("\t- ",'red','bold');
        echo '#[Command("'.$method1->getCommand().'")]';
        echo PHP_EOL;
        $this->color("\t+ ",'green','bold');
        echo '#[Command("'.$method1->getCommand().'_1")]';
        echo PHP_EOL;

    }

    public function correction(): void
    {
        match($this->binError->errorType) {
            Error::DoubleCommand => $this->doubleCommandError($this->binError->method1,$this->binError->method2),
        };
    }

}
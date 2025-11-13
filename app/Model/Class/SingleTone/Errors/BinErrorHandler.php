<?php

namespace Stampy\Model\Class\SingleTone\Errors;


use Stampy\Model\Class\Throwable\BinError;
use Stampy\Model\Interface\SingleToneInterface;
use Stampy\Model\Trait\Coloring;
use Stampy\Model\Enum\Error;
use Stampy\Model\Interface\MethodCLIInterface;
use Stampy\Model\Abstract\AbstractError;
use StampyException;


class BinErrorHandler extends AbstractError implements SingleToneInterface {
    
    use Coloring;
    private static ?BinErrorHandler $instance;

    private function __construct() {
        parent::__construct();
    }
    
    private function __clone(){}

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
        $input = "";
        $input .= $this->textColor("You have double Command Name",'red','bold');
        $input .= PHP_EOL;
        $info = function(?MethodCLIInterface $method = null) use (&$input){
            $input .= $this->textColor("In file " ,'blue');
            $input .= $this->textColor($method->getFile() ,'bgblue');
            $input .= PHP_EOL;
            $input .= $this->textColor($method->getClass()."()->".$method->getName().'()','yellow','underline');
            $input .= $this->textColor(" in line ",'yellow');
            $input .= $this->textColor($method->getLine(),'yellow','underline');
            $input .= PHP_EOL;
        };
        $info($method1);
        $info($method2);
        $input .= PHP_EOL;
        $input .= $this->textColor("You might want to change one of those two command Attribut:",'green','underline');
        $input .= PHP_EOL;
        $input .= $this->textColor("\t- ",'red','bold');
        $input .= '#[Command("'.$method1->getCommand().'")]';
        $input .= PHP_EOL;
        $input .= $this->textColor("\t+ ",'green','bold');
        $input .= '#[Command("'.$method1->getCommand().'_1")]';
        $input .= PHP_EOL;
        $this->stderr_tty->write($input);
    }

    private function classNotFound(string $className){
        $input = "";
        $arrayNamespace = explode("\\",$className);
        $basename = end($arrayNamespace);
        $file = getenv("ENTRY").$basename.".php";
        $input .= $this->textColor("Class $className not found.",'red','bold');
        $input .= PHP_EOL;
        $input .= $this->textColor("that mean the file '$className.php' might exist but the class '$basename' don't." ,'blue');
        $input .= PHP_EOL;
        $input .= $this->textColor("Or that the namespace attach to it might be not correctly saved in your " ,'blue');
        $input .= $this->textColor("composer.json" ,'blue','underline');
        $input .= $this->textColor(" file" ,'blue');

        $input .= PHP_EOL;
        $input .= $this->textColor("You might want to add in the file '".$file. "' the class '$basename':" ,'green','underline');
        $input .= PHP_EOL;
        $input .= $this->textColor("\t+ ",'green','bold');
        $input .= "class $basename { }";
        $input .= PHP_EOL;

          $input .= $this->textColor("\t+ ",'green','bold');
        $input .= "Or check your composer.json file ";
        $input .= PHP_EOL;
        $this->stderr_tty->write($input);
    }

    public function correctionInternalError(BinError $binError): void
    {
        match($binError->errorType) {
            Error::DoubleCommand => $this->doubleCommandError($binError->method1,$binError->method2),
            Error::ClassNotFound => $this->classNotFound($binError->className),
        };
    }

    public function correctionStampyError(\StampyException $stampyException): void
    {
        $stampyException->getFormatMessage();
    }

}
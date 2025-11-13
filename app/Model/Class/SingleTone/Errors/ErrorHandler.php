<?php

namespace Stampy\Model\Class\SingleTone\Errors;

use ConsoleTTY;
use Stampy\Model\Interface\SingleToneInterface;
use Stampy\Model\Trait\Coloring;
use Error;
use Stampy\Model\Class\SingleTone\StampyConsole;

class ErrorHandler implements SingleToneInterface {
    use Coloring;
    private static ?ErrorHandler $instance;
    private ConsoleTTY $stderr_tty;

    private function __construct(
        private null|Error $error=null,
    ) {
        $stampyConsoleOInstance = &StampyConsole::instance();
        $this->stderr_tty = $stampyConsoleOInstance->getStdErrTTY();
    }

    
    private function __clone()
    {}

    public function __wakeup()
    {
        throw new \Exception("Cannot unserialize a singleton.");
    }

    public function init(mixed ...$args): void{
        self::$instance =  new static(... $args);
    }

    public static function &instance(mixed ...$args):ErrorHandler
    {   
        {
            if(!isset(self::$instance))
            {
                self::$instance =  new static(... $args);
            }
            return self::$instance;
        }
    }

    public function debugInfo(): void 
    {
        $this->colorErr($this->error->getMessage(),"red","bold");
        $this->stderr_tty->write(PHP_EOL);
        $this->colorErr("In file:".$this->error->getFile()." ","bgblue","bold");
        $this->colorErr("on line:".$this->error->getLine(),"bgblue","underline","bold");
        $this->stderr_tty->write(PHP_EOL);
        $this->contextError();
        $this->stderr_tty->write(PHP_EOL);
        $this->colorErr("stack trace:","yellow","underline","bold");
        $this->stderr_tty->write(PHP_EOL);
        foreach($this->error->getTrace() as $trace) {
            $this->colorErr(($trace["class"] ?? "").($trace["type"] ?? "").($trace["function"]??""),"yellow","underline","italic");
            if(isset($trace["file"])){
                $this->stderr_tty->write(PHP_EOL);
                $this->stderr_tty->write("in file");
            $this->stderr_tty->write(PHP_EOL);
                $this->colorErr($trace["file"],"bgmagenta","bold");
            }
            if(isset($trace["line"])){
                $this->colorErr(" on line ".$trace["line"],"magentabg","bold");
            }
            $this->stderr_tty->write(PHP_EOL);
        }
        $this->stderr_tty->write(PHP_EOL);
        $this->stderr_tty->flush();
    }
    
    protected function contextError():void {
        $i = 0;
        $this->colorErr("Error context :",'green','bold');
        $this->stderr_tty->write(PHP_EOL);
        $this->colorErr(str_repeat("─", 80),'green');
        $this->stderr_tty->write(PHP_EOL);
        if($file = fopen($this->error->getFile(), "r")){
            // If file is open
            while(($line=fgets($file)) !== false){
                $i++;
                $superiorCondition = $i < $this->error->getLine() + 5;
                if($i > $this->error->getLine() -5 && $superiorCondition)
                {
                        $linePrefix = "[Line #{$i}]:";
                        if($i != $this->error->getLine()){
                            $this->colorErr($linePrefix,'green','italic');
                            $this->stderr_tty->write($line);
                        } else {
                            $this->colorErr($linePrefix,'red','italic');
                            $this->colorErr($line,'red','italic','bold');
                        }  
                }
                if(!$superiorCondition){
                    break;
                }
            }
            fclose($file);
        }
        $this->stderr_tty->write(PHP_EOL);
        $this->colorErr(str_repeat("─", 80),'green');
    }
}
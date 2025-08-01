<?php

namespace Stampy\Model\Class\Singletone;

use Stampy\Model\Class\throwable\binError;
use Stampy\Model\Interface\SingleToneInterface;
use Stampy\Model\Trait\Coloring;
use Error;

class ErrorHandler implements SingleToneInterface {
    use Coloring;
    private static ?ErrorHandler $instance;

    private function __construct(
        private null|Error $error=null,
    ) {}

    public function __destruct()
    {
        echo PHP_EOL;
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
    $this->color($this->error->getMessage(),"red","bold");
    echo PHP_EOL;
    $this->color("In file:".$this->error->getFile()." ","bgblue","bold");
    $this->color("on line:".$this->error->getLine(),"bgblue","underline","bold");
    echo PHP_EOL;
    $this->contextError();
    echo PHP_EOL;
    $this->color("stack trace:","yellow","underline","bold");
        echo PHP_EOL;
    foreach($this->error->getTrace() as $trace){
        $this->color($trace["class"].$trace["type"].$trace["function"],"yellow","underline","italic");
        if(isset($trace["file"])){
        echo PHP_EOL;
            echo "in file ";
        echo PHP_EOL;
            $this->color($trace["file"],"bgmagenta","bold");
        }
        if(isset($trace["line"])){
            $this->color(" on line ".$trace["line"],"magentabg","bold");
        }
        echo PHP_EOL;
    }
    echo PHP_EOL;
    }

    protected function contextError():void {
        $i = 0;
        $this->color("Error context :",'green','bold');
        echo PHP_EOL;
        $this->color(str_repeat("─", 80),'green');
        echo PHP_EOL;
        if($file = fopen($this->error->getFile(), "r")){
            // If file is open
            while(($line=fgets($file)) !== false){
                $i++;
                $superiorCondition = $i < $this->error->getLine() + 5;
                if($i > $this->error->getLine() -5 && $superiorCondition)
                {
                        $linePrefix = "[Line #{$i}]:";
                        if($i != $this->error->getLine()){
                            $this->color($linePrefix,'green','italic');
                            echo $line;
                        } else {
                            $this->color($linePrefix,'red','italic');
                            $this->color($line,'red','italic','bold');
                        }  
                }
                if(!$superiorCondition){
                    break;
                }
            }
            fclose($file);
        }
        $this->color(str_repeat("─", 80),'green');
    }
}
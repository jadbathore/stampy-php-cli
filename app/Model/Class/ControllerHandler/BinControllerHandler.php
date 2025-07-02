<?php

namespace App\Model\Class\ControllerHandler;

use \Error;
use \Generator;
use App\Model\Class\IteratorAggregate\ClassAttributHandler_CLI;
use App\Model\Attributes\Command;
use App\Model\Class\IteratorAggregate\RaisedmethodHandler_CLI;
use App\Model\Class\Object\Argv_CLI;
use ReflectionClass;
use App\Model\Class\Object\Method_CLI;
use App\Model\Interface\MethodCLIInterface;
use App\Model\Enum\Argv;
use App\Model\Trait\Coloring;

class BinControllerHandler
{
    use Coloring;

    private ClassAttributHandler_CLI $classAtributHandlerIterator;
    private RaisedmethodHandler_CLI $raisedMethodIterator;
    private Argv_CLI $argvObject;

    public function __construct(
        private string $controller,
        array $argv,
    ) 
    {
        $this->argvSetter($argv);
        $this->setAttributIterator();
        $this->setRaisedmethodIterator();
        if($this->argvObject->length == 0)
        {
            $this->invokeDebuggingMethod();
        }

    }

    private function setAttributIterator()
    {
        $reflec_class = new ReflectionClass($this->controller);
        $this->classAtributHandlerIterator = new classAttributHandler_CLI($reflec_class);
        foreach($reflec_class->getMethods() as $method)
        {
            if(!empty($method->getAttributes(Command::class)))
            {
                $methodHandler = new method_CLI($method);
                $this->classAtributHandlerIterator->addItem($methodHandler);
            }
        }
    }

    private function setRaisedmethodIterator()
    {
        $this->raisedMethodIterator = new raisedmethodHandler_CLI();
    }

    private function argvSetter(array $argv)
    {

        $this->argvObject = new argv_CLI((count($argv)<=1)?$argv:array_slice($argv,1));
    }

    public function start():void
    {

        foreach($this->classAtributHandlerIterator->getIterator() as $method_CLI) 
        {
            if($method_CLI->getCommand() == $this->argvObject->getCurrent())
            {
                $this->argvObject->next();
                while($this->argvObject->isValid())
                {
                    switch($this->argvObject->currentArgvType($method_CLI))
                    {
                        case Argv::Option:
                            $method_CLI->addPromps($this->argvObject->getCurrent(),true);
                        break;
                        case Argv::Input:
                            $method_CLI->addPromps($this->argvObject->getLast(),$this->argvObject->getCurrent());
                        break;
                        default: throw new Error("unknown Option '".$this->argvObject->getCurrent()."'");
                    } 
                    $this->argvObject->next();
                } 
                $this->raisedMethodIterator->addItem($method_CLI);
                if(!$this->argvObject->isValid())
                {
                    break 1;
                }
            }
        }

        $this->invokeAllRaisedMethod();
    }


    private function invokeAllRaisedMethod():void
    {
        if(!empty($this->raisedMethodIterator->getItems()))
        {
            foreach($this->raisedMethodIterator->getIterator() as $raisedMethod)
            {
                $raisedMethod->invokeFromPromps();

            } 
        } else {
            $this->invokeDebuggingMethod();
        }
    }

    /**
     * @return \Generator<TKey, MethodCLIInterface>| MethodCLIInterface[]
     */
    private function generateCommand():Generator {
        foreach($this->classAtributHandlerIterator->getIterator() as $method_CLI)
        {
            if($this->argvObject->currentArgvType($method_CLI) == Argv::Command){
                yield $method_CLI;
            }
        }
    }

    private function defaultDebugScript(string $color = "green"):void
    {
        foreach($this->classAtributHandlerIterator->getIterator() as $method_CLI)
        {
            $this->color(str_repeat("=", 80)."\n",$color);
            $method_CLI->method_debug_script($color);
        }
        $this->color(str_repeat("=", 80)."\n",$color);
    }

    private function invokeDebuggingMethod():void
    {
        if(!is_null($this->classAtributHandlerIterator->getDebbugingMethod()))
        {
            $this->classAtributHandlerIterator->getDebbugingMethod()->invoke($this->defaultDebugScript(...));
        } else {
            $this->defaultDebugScript();
        }
    }
}

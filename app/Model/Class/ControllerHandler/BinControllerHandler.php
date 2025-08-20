<?php

namespace Stampy\Model\Class\ControllerHandler;

use \Error;
use Stampy\Model\Class\IteratorAggregate\ClassAttributHandler_CLI;
use Stampy\Model\Attributes\Command;
use Stampy\Model\Class\IteratorAggregate\ControllerHandler_CLI;
use Stampy\Model\Class\IteratorAggregate\RaisedmethodHandler_CLI;
use Stampy\Model\Class\Object\Argv_CLI;
use ReflectionClass;
use Stampy\Model\Abstract\abstractControllerHandler;
use Stampy\Model\Abstract\abstractPrompsController;
use Stampy\Model\Class\Object\Method_CLI;
use Stampy\Model\Class\throwable\binError;
use Stampy\Model\Interface\MethodCLIInterface;
use Stampy\Model\Enum\Argv;
use Stampy\Model\Enum\Error as EnumError;
use Stampy\Model\Trait\Coloring;

class BinControllerHandler extends AbstractControllerHandler
{
    use Coloring;

    private RaisedmethodHandler_CLI $raisedMethodIterator;
    private ControllerHandler_CLI $controllerHandlerIterator;
    private ?methodCLIInterface $debuggingMethod;

    public function __construct(
        private array $controllers,
        array $argv,
    ) 
    {
        $this->controllerHandlerIterator = new ControllerHandler_CLI();
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
        $tempDoubleCheck = [];
        foreach ($this->controllers as $controller) {
            $reflec_class = new ReflectionClass($controller);
            $classAtributHandlerIterator = new classAttributHandler_CLI($reflec_class);
            foreach($reflec_class->getMethods() as $method)
            {
                if(!empty($method->getAttributes(Command::class)))
                {
                    $methodHandler = new method_CLI($method);
                    if(!in_array($methodHandler->getCommand(),$tempDoubleCheck)) {
                        $classAtributHandlerIterator->addItem($methodHandler);
                        $tempDoubleCheck[] = $methodHandler->getCommand();
                    } else {
                        throw new binError(
                            EnumError::DoubleCommand,
                            $method,
                            $this->controllerHandlerIterator->getmethod($methodHandler->getCommand()),
                        );
                    }
                }
            }
            $this->controllerHandlerIterator->addItem($classAtributHandlerIterator);
        }
    }

    private function setRaisedmethodIterator()
    {
        $this->raisedMethodIterator = new raisedmethodHandler_CLI();
    }

    protected function argvSetter(array $argv)
    {
        $this->argvObject = new argv_CLI((count($argv)<=1)?$argv:array_slice($argv,1));
    }

    protected function populateMethod(Method_CLI $method_CLI)
    {
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
    }


    public function start():void
    {
        foreach($this->controllerHandlerIterator->generateMethod() as $method_CLI)
        {
            if($method_CLI->getCommand() == $this->argvObject->getCurrent())
            {
                if($method_CLI->useSTD()){
                    echo implode(" ",[
                    "EXIT",
                    $method_CLI->getClass(),
                    $method_CLI->getName(), 
                    $method_CLI->getStdErr() ?? "#",
                    $method_CLI->getStdIn() ?? "#",
                    $method_CLI->getStdOut()?? "#" 
                    ]);
                    die;
                }
                $this->argvObject->next();
                $this->populateMethod($method_CLI);
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

    private function defaultDebugScript(string $color = "green"):void
    {
        foreach ($this->controllerHandlerIterator->generateMethod() as $method_CLI) {
            $this->color(str_repeat("=", 80)."\n",$color);
            $method_CLI->method_debug_script($color);
        }
        $this->color(str_repeat("=", 80)."\n",$color);
    }

    private function invokeDebuggingMethod():void
    {
        $nullableDebuggingMethod = $this->controllerHandlerIterator->getDebbugingMethod();
        if (!is_null($nullableDebuggingMethod))
        {
            $nullableDebuggingMethod->invoke($this->defaultDebugScript(...));
        } else {
            $this->defaultDebugScript();
        }
    }
}

<?php

namespace Stampy\Model\Class\ControllerHandler;

use ReflectionClass;
use Stampy\Model\Abstract\AbstractControllerHandler;
use Stampy\Model\Class\Object\Method_CLI;


class JumpStart extends AbstractControllerHandler
{
    private Method_CLI $method;

    public function __construct(
        string $className,
        string $methodName,
        array $argv
    ) 
    {
        $refClass = new ReflectionClass($className);
        $this->method = new Method_CLI($refClass->getMethod($methodName));
        $this->argvSetter($argv);
    }

    public function start():void
    {
        $this->populateMethod($this->method);
        $this->method->invokeFromPromps();
    }
    
}

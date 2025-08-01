<?php

namespace Stampy\Model\Class\IteratorAggregate;

use Stampy\Model\Iterator\ControllerIterator;
use Stampy\Model\Class\IteratorAggregate\ClassAttributHandler_CLI;
use \IteratorAggregate;
use Stampy\Model\Interface\MethodCLIInterface;
use IteratorIterator;

class ControllerHandler_CLI implements \IteratorAggregate
{
    private $items = [];
    private ?methodCLIInterface $debuggingMethod;

    public function getItems()
    {
        return $this->items;
    }

    public function addItem(ClassAttributHandler_CLI $item)
    {
        $this->items[] = $item;
    }

    /**
     * @return \Traversable<TKey, ClassAttributHandler_CLI>|ClassAttributHandler_CLI[]
     */
    public function getIterator(): \Iterator
    {
        return new ControllerIterator($this);
    }

    /**
     * @return \Traversable<TKey, ClassAttributHandler_CLI>|ClassAttributHandler_CLI[]
     */
    public function getReverseIterator(): \Iterator
    {
        return new ControllerIterator($this, true);
    }

    /**
     * @return \Generator<TKey, MethodCLIInterface>| MethodCLIInterface[]
     */
    public function generateMethod():\Generator {
        
        foreach(new ControllerIterator($this) as $controller)
        {
            foreach($controller->getIterator() as $method_CLI)
            {
                yield $method_CLI;
            }
        }
    }

    public function getmethod(String $commandName):?MethodCLIInterface
    {
        /**
         * @return \Traversable<TKey, MethodCLIInterface>|MethodCLIInterface[]
         */
        $test = function() use ($commandName):\Generator{
            foreach($this->generateMethod() as $method){
                if ($commandName == $method->getCommand()){
                    yield $method;
                }
            }
        };
        return $test()->current();
    }

    public function getDebbugingMethod():?methodCLIInterface
    {
        if(!isset($this->debuggingMethod))
        {
            $debuggingMethod = null;
            foreach($this->generateMethod() as $method_CLI)
            {
                if($method_CLI->getCommand() == 'debug')
                {
                    $debuggingMethod = $method_CLI;
                    break;
                }
            }
            $this->debuggingMethod = $debuggingMethod;
        }
        return $this->debuggingMethod;
    }


}
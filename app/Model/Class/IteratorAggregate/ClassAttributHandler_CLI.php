<?php

namespace Stampy\Model\Class\IteratorAggregate;

use Stampy\Model\Iterator\AttributIterator_CLI;
use \IteratorAggregate;
use Stampy\Model\Interface\MethodCLIInterface;

class ClassAttributHandler_CLI implements IteratorAggregate
{
    private $items = [];
    private ?methodCLIInterface $debuggingMethod;

    public function __construct(private \ReflectionClass $method) 
    {}

    public function getItems()
    {
        return $this->items;
    }

    public function addItem(methodCLIInterface $item)
    {
        $this->items[] = $item;
    }

    /**
     * @return \Traversable<TKey, MethodCLIInterface>|MethodCLIInterface[]
     */
    public function getIterator(): \Iterator
    {
        return new AttributIterator_CLI($this);
    }

    /**
     * @return \Traversable<TKey, MethodCLIInterface>|MethodCLIInterface[]
     */
    public function getReverseIterator(): \Iterator
    {
        return new AttributIterator_CLI($this, true);
    }

    /**
     * @return \Traversable<TKey, MethodCLIInterface>|MethodCLIInterface[]
     */
    private function getGeneratedMethod(String $className):\Generator
    {
        var_dump($this->items);
        foreach($this->getIterator() as $method){
            var_dump($method);
            if ($className == $method->getName()){
                yield $method;
                // break;
            }
        }
    }

    public function getmethod(String $className):?MethodCLIInterface
    {
        foreach($this->getGeneratedMethod($className)as $a){
            var_dump($a);
        }
        return null;
    }
}
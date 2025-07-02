<?php

namespace App\Model\Class\IteratorAggregate;

use App\Model\Iterator\AttributIterator_CLI;
use \IteratorAggregate;
use App\Model\Interface\MethodCLIInterface;

class classAttributHandler_CLI implements IteratorAggregate
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

    public function getDebbugingMethod():?methodCLIInterface
    {
        if(!isset($this->debuggingMethod))
        {
            $debuggingMethod = null;
            foreach($this->getIterator() as $method_CLI)
            {
                if($method_CLI->getCommand() == 'debug')
                {
                    $debuggingMethod = $method_CLI;
                }
            }
            $this->debuggingMethod = $debuggingMethod;
        }
        return $this->debuggingMethod;
    }
}
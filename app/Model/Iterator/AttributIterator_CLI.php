<?php

namespace Stampy\Model\Iterator;

use Stampy\Model\Class\IteratorAggregate\ClassAttributHandler_CLI;
use Stampy\Model\Class\IteratorAggregate\RaisedmethodHandler_CLI;
use Stampy\Model\Interface\MethodCLIInterface;

class AttributIterator_CLI implements \Iterator
{
    private int $index = 0;

    public function __construct(
        private classAttributHandler_CLI|raisedmethodHandler_CLI $collection,
        private bool $reverse = false
        )
    {
    }

    public function rewind():void
    {
        $this->index = $this->reverse ? count($this->collection->getItems()) - 1 : 0;
    }

    public function current():methodCLIInterface
    {
        return $this->collection->getItems()[$this->index];
    }

    public function key():int
    {
        return $this->index;
    }

    public function next():void
    {
        $this->index = $this->index + ($this->reverse ? -1 : 1);
    }

    public function valid():bool
    {
        return isset($this->collection->getItems()[$this->index]);
    }
    
}
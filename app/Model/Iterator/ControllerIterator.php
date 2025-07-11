<?php

namespace App\Model\Iterator;

use App\Model\Class\IteratorAggregate\ClassAttributHandler_CLI;
use App\Model\Class\IteratorAggregate\ControllerHandler_CLI;
use App\Model\Class\IteratorAggregate\RaisedmethodHandler_CLI;
use App\Model\Interface\MethodCLIInterface;

class ControllerIterator implements \Iterator
{
    private int $index = 0;

    public function __construct(
        private ControllerHandler_CLI $collection,
        private bool $reverse = false
        )
    {}

    public function rewind():void
    {
        $this->index = $this->reverse ? count($this->collection->getItems()) - 1 : 0;
    }

    public function current():ClassAttributHandler_CLI
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
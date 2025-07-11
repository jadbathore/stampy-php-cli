<?php

namespace App\Model\Class\IteratorAggregate;

use App\Model\Iterator\ControllerIterator;
use App\Model\Class\IteratorAggregate\ClassAttributHandler_CLI;
use \IteratorAggregate;
use App\Model\Interface\MethodCLIInterface;
use IteratorIterator;

class ControllerHandler_CLI implements \IteratorAggregate
{
    private $items = [];

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
}
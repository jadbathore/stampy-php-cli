<?php

namespace App\Model\Class\IteratorAggregate;

use App\Model\Iterator\AttributIterator_CLI;
use \IteratorAggregate;
use App\Model\Interface\MethodCLIInterface;
use \Iterator;

class RaisedmethodHandler_CLI implements IteratorAggregate
{
    private $items = [];

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
    public function getIterator(): Iterator
    {
        return new AttributIterator_CLI($this);
    }

    /**
     * @return \Traversable<TKey, MethodCLIInterface>|MethodCLIInterface[]
     */
    public function getReverseIterator(): Iterator
    {
        return new AttributIterator_CLI($this, true);
    }
}
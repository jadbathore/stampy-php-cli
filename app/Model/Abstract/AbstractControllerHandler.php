<?php

namespace Stampy\Model\Abstract;
use Stampy\model\class\Object\Argv_CLI;

abstract class abstractControllerHandler 
{
    protected Argv_CLI $argvObject;

    abstract public function start():void;
}

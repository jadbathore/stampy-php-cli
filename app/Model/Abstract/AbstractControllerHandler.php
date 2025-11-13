<?php

namespace Stampy\Model\Abstract;
use Stampy\Model\Class\Object\Argv_CLI;
use Stampy\Model\Class\Object\Method_CLI;
use Stampy\Model\Class\SingleTone\StampyConsole;
use Stampy\Model\Enum\Argv;

abstract class abstractControllerHandler 
{
    protected Argv_CLI $argvObject;
    protected StampyConsole $stampyConsole;

    public function __construct() {
        $this->stampyConsole = &StampyConsole::instance();
    }

    abstract public function start():void;

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
                default: throw new \Error("unknown Option '".$this->argvObject->getCurrent()."'");
            } 
            $this->argvObject->next();
        } 
    }

}

<?php

namespace App\model\class\Object;

use App\Model\Enum\Argv;
use App\Model\Interface\ArgvInterface;
use App\Model\Interface\MethodCLIInterface;

class Argv_CLI implements ArgvInterface {

    private int $index = 0;
    public int $length;

    public function __construct(
        private array $argv
    )
    {
        $this->length = count($this->argv);
    }

    public function getCurrent():string
    {
        return ($this->isValid())?$this->argv[$this->index]:$this->argv[$this->length -1];
    }

    public function isValid():bool
    {
        return isset($this->argv[$this->index]);
    }

    public function getNext():string
    {
        return $this->argv[($this->nextIsValid())?$this->index+1:$this->length - 1];
    }

    public function getLast():string
    {
        return $this->argv[($this->lastIsValid())?$this->index-1:0];
    }

    

    public function nextIsValid():bool
    {
        return isset($this->argv[$this->index + 1]);
    }
    public function lastIsValid():bool
    {
        return isset($this->argv[$this->index - 1]);
    }


    public function currentArgvType(MethodCLIInterface $methodCLI):Argv
    {
        return Argv::type_Argv($this,$methodCLI);
    }

    public function next():void
    {
        $this->index++;
    }
    public function getPrompsArgv():string
    {
        return implode(' ',$this->argv);
    }
}
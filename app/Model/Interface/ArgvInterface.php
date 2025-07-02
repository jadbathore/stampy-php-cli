<?php

namespace App\Model\Interface;

use App\Model\Enum\Argv;

interface ArgvInterface {
    public function getCurrent():string;
    public function getNext():string;
    public function getLast():string;
    public function isValid():bool;
    public function nextIsValid():bool;
    public function lastIsValid():bool;
    public function currentArgvType(methodCLIInterface $methodCLIInterface):Argv;
    public function getPrompsArgv():string;
    public function next():void;
}
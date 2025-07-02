<?php

namespace App\Model\Interface;

interface MethodCLIInterface {
    public function getOptions(): null|array;
    public function getCommand(): string;
    public function getName():string;
    public function getClass():string;
    public function addPromps(mixed $index,mixed $item):void;
    public function getPromps():null|array;
    public function invokeFromPromps():void;
    public function invoke():void;
    public function getDescription():null|string;
    public function method_debug_script(string $color):void;
}
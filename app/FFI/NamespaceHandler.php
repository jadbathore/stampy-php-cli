<?php

interface NamespaceHandler {
    public function __construct(string $input,string $namespace);
    public function resolve():Array;
    public function previous():void;
}
<?php


interface NamespaceHandler {
    public function __construct(string $input,string $namespace);
    public function getAssocitedClass():Array;
    public function getNamespace():String;
}
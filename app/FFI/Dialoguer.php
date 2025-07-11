<?php

interface Dialoguer {
    public static function confirm(string $input,?Bool $theme=false):bool;
    public static function input(string $input,?Bool $theme=false):string;
    public static function password(string $input,?Bool $theme=false):void;
    /**
     * @param string[] $list
     */
    public static function select(string $input,array $list,?Bool $theme=false):string;
    /**
     * @param string[] $list
     */
    public static function multiSelect(string $input,array $list,?Bool $theme=null):Array;
    public static function editor(string $input):void;
}
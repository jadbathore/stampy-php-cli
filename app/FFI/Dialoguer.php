<?php

interface Dialoguer {
    public static function confirm(string $input,?Bool $theme=false);
    public static function input(string $input,?Bool $theme=false);
    public static function password(string $input,?Bool $theme=false);
    /**
     * @param string[] $list
     */
    public static function select(string $input,array $list,?Bool $theme=false);
    /**
     * @param string[] $list
     */
    public static function multiSelect(string $input,array $list,?Bool $theme=null);
}
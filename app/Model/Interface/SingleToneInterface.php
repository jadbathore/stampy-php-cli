<?php

namespace App\Model\Interface;

interface SingleToneInterface {
    public function __wakeup();
    public function init(mixed ...$args):void;
    public static function &instance(mixed ...$args):self;
}
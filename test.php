#!/usr/bin/env php
<?php

require_once __DIR__.'vendor/autoload.php';

use Stampy\Model\Class\ControllerHandler\BinControllerHandler;
use Stampy\Model\Class\ControllerHandler\JumpStart;
use Stampy\Model\Class\SingleTone\ErrorHandler;
use Stampy\Model\Class\SingleTone\BinErrorHandler;
use Stampy\Model\Class\throwable\binError;

try{
    if(getenv("CLASS") && getenv("METHOD")){
     
        new JumpStart(getenv("CLASS"),getenv("METHOD"),array_slice($argv,1))->start();
    } else {
        $controllersNameSpace = new \NamespaceHandler(__DIR__.getenv("ENTRY"),getenv("NAMESPACE"));
        // var_dump(__DIR__.getenv("ENTRY"),getenv("NAMESPACE"),$controllersNameSpace->resolve());
        
        new BinControllerHandler($controllersNameSpace->resolve(),$argv)->start();
    }
} catch(Error $e) {
    $errorHandler = &ErrorHandler::instance($e);
    $errorHandler->debugInfo();
} catch(binError $e) {
    $errorHandler = &BinErrorHandler::instance($e);
    $errorHandler->correction();
}



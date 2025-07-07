#!/usr/bin/env php
<?php 
require_once 'vendor/autoload.php';

// require_once './public/ModelAutoloader.php';

use App\Controller;
use App\Model\Class\ControllerHandler\BinControllerHandler;
use App\Model\Class\Singletone\ErrorHandler;

// try{
//     $action = new BinControllerHandler(BinController::class,$argv);
//     $action->start();
// }catch(Error $e){
//     $errorHandler = &ErrorHandler::instance($e);
//     $errorHandler->debugInfo();
// }


var_dump(NamespaceHandler::getAssocitedClass(__DIR__."/app","App\\Controller"));

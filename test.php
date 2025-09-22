<?php

require_once __DIR__.'vendor/autoload.php';

use Stampy\Model\Class\ControllerHandler\BinControllerHandler;
use Stampy\Model\Class\ControllerHandler\JumpStart;
use Stampy\Model\Class\Singletone\ErrorHandler;
use Stampy\Model\Class\Singletone\BinErrorHandler;
use Stampy\Model\Class\throwable\binError;


echo "BinControllerHandler ". class_exists("Stampy\Model\Class\ControllerHandler\BinControllerHandler");
echo "JumpStart". class_exists("Stampy\Model\Class\ControllerHandler\JumpStart");
echo "ErrorHandler". class_exists("Stampy\Model\Class\Singletone\ErrorHandler");
echo "BinErrorHandler". class_exists("Stampy\Model\Class\Singletone\BinErrorHandler");
<?php

namespace App\Model\Class\throwable;

use App\Model\Class\Object\Method_CLI;
use App\Model\Enum\Error;
use App\Model\Interface\MethodCLIInterface;
use Exception;
use ReflectionMethod;
use Throwable;

class binError extends Exception {

    public private(set) Error $errorType;
    public private(set) Method_CLI $method1;
    public private(set) ?Method_CLI $method2;

    public function __construct(
        Error $errorType,
        ReflectionMethod $method1,
        ?MethodCLIInterface $method2 = null,
        int $code = 0 ,
        ?Throwable $throwable =null)
    {
        $this->errorType = $errorType;
        parent::__construct("",$code,$throwable);
        $this->method1 = new Method_CLI($method1);
        $this->method2 = $method2;
    }
}
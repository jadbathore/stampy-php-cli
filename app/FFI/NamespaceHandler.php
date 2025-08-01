<?php

interface NamespaceHandler {

    /**
     * @param string $path the path base of the namespace 
     * @param string $namespace the actual namespace 
     * (you could use App\Controller::class for example even if it's not a actual class)
     */
    public function __construct(string $path,string $namespace);
    /**
     * get all associated sub namespace for this namespace for example you add (App\Controller::class)
     * ```
     *  app
     *  ├── controller
     *  │       ├── controller1.php
     *  │       └── controller2.php
     *  ├── FFI
     *  └── src
     * ```
     * 
     * ```
     *  $namespace = new NamespaceHandler(App\Controller::class);
     *  $namespace->resolve();// ["App\Controller\controller1","App\Controller\controller2"]
     * ```
     */
    public function resolve():Array;

    /**
     * get all previous namespace for this namespace for example you add (App\Controller::class)
     * previous make your namespace be \App so for 
     * ```
     *  app
     *  ├── controller
     *  │       ├── controller1.php
     *  │       └── controller2.php
     *  ├── FFI
     *  └── src
     * ```
     * ```php
     *  $namespace = new NamespaceHandler(App\Controller::class);
     *  $namespace->resolve();// ["App\Controller\controller1","App\Controller\controller2"]
     *  $namespace->previous();
     *  $namespace->resolve(); // ["App\Controller","App\FFI","App\src"]
     * ```
     *  if you came to the root of your namespace define by path you will add always the namespace root even if you call previous multiple time 
     */
    public function previous():void;

    /**
     * push (App\Controller::class)
     * previous make your namespace be \App so for 
     * ```
     *  app
     *  ├── controller
     *  │       ├── controller1.php
     *  │       └── controller2.php
     *  ├── FFI
     *  └── src  
     * ```
     * ```php
     *  $namespace = new NamespaceHandler(\App::class);
     *  $namespace->resolve();// ["App\Controller","App\FFI","App\src"]
     *  $namespace->push("controller");
     *  $namespace->resolve(); // ["App\Controller\controller1","App\Controller\controller2"]
     *  $namespace->push("model"); // will throw a error "namespace App\controller\model" don't exist 
     * ```
     *  if you came to the root of your namespace define by path you will add always the namespace root even if you call previous multiple time
     */
    public function push(string $namespaceSlice):void;
}
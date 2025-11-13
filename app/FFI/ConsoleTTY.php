<?php

interface ConsoleTTY
{

    /**
     * To build a ConsoleTTY object you must use the const Stampy\STDERR_KEY or Stampy\STDOUT_KEY
     * @param string $terminal if you want to use the tty terminal to a just ouput thing in your terminal use Stampy\STDOUT_KEY
     * or if you want to use the console to buffer,flush and exit with a error use Stampy\STDOUT_KEY
     *  ```
     *  use const Stampy\STDOUT_KEY;
     *  use const Stampy\STDERR_KEY;
     *  
     *  $out = new ConsoleTTY(STDOUT_KEY);
     *  $out->write("hello,world !"); // will right directly in your terminal even if you use a redirection
     *  $err = new ConsoleTTY(STDERR_KEY);
     *  $err->write("hello,world !"); // will bufferize all your input nothing will appear if you don't flush it
     *  $err->flush(); // this will flush all your input an then exit your code with code 1 so this will be condider a error 
     * ```
     */
    public function __construct(string $terminal);
    /** 
     * If you want to add a read-write pipeline use consoleTTY with 2 param.
     * @param ?string $inputFile will be the path of the file you want to use as an stdin or use Stampy\STDIN_KEY
     * @param ?string $terminal Stampy\STDERR_KEY or Stampy\STDOUT_KEY  
     * if you want to add a read-write pipeline the consoleTTY with 2 param. on this param Stampy\STDERR_KEY or Stampy\STDOUT_KEY  
     * ```
     *  use const Stampy\STDOUT_KEY;
     *  use const Stampy\STDERR_KEY;
     *  use const Stampy\STDIN_KEY;
     *  
     *  $readWriteoutput = new ConsoleTTY(STDIN_KEY,STDOUT_KEY);
     *  $readWriteoutput->write("hello,world !"); 
     *  //you can still use the ConsoleTTY like so 
     *  //but the stdin you alwaly be your terminal even if you redirect 
     *  $readWriteErrorOutput = new ConsoleTTY("inputfile.txt",STDERR_KEY);
     *  // if you don't use a path you Stdin for that pipe you be "inputfile.txt" 
     *  $readWriteErrorOutput->write("hello,world !"); // will bufferize all your input nothing will appear if you don't flush it
     *  $readWriteErrorOutput->flush(); // this will flush all your input an then exit your code with code 1 so this will be condider a error 
     * ```
     */
    public function __construct(string $inputFile,?string $terminal);
    /**
     * @param string $input will output or buffer your input depending on the STD named at the class construction;
     */
    public function write(string $input);
    /**
     * will flush all your input work only if you using the the const Stampy\STDERR_KEY using flush 
     * if you use Stampy\STDOUT_KEY that will throw a fatal error
     * 
     * ```
     *  use const Stampy\STDOUT_KEY;
     *  use const Stampy\STDERR_KEY;
     *  
     *  $err = new ConsoleTTY(STDERR_KEY);// use this 
     *  $out = new ConsoleTTY(STDOUT_KEY);// not this 
     * ```
     */
    public function flush();
} 
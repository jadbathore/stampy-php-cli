<?php

/**
 * ## Dialoguer
 * An implementation of the Rust crate "Dialoguer" allowing for various interactions with the terminal.
 */
interface Dialoguer {
    /**
     * ## confirm 
     * allow to add a confirm in the terminal 
     *  
     * ```php
     *  $confirm = Dialoguer::confirm("are you sure ?");
     *  // the terminal will ask you "are you sure ?" 
     *  // you pick true or false
     *  if $confirm {
     *      echo "ok"; // output true
     *  } else {
     *      echo "fortunately, we check"; // output false 
     *  }
     * ```
     * @param string $input confirmation question before interaction
     * @param bool $theme optionnal $theme with default="false" colorful="true"
     */
    public static function confirm(string $input,?Bool $theme=false):bool;

    /**
     * ## input 
     * allow to add a input in the terminal 
     *  
     * ```php
     *  $name = Dialoguer::input("what's your name ?");
     *  // the terminal will ask you "what's your name ?" 
     *  // you add the input "marie"
     *  echo "welcome back $name" // welcome back marie
     * ```
     * @param string $input input question before interaction
     * @param bool $theme optionnal $theme with default="false" colorful""
     */
    public static function input(string $input,?Bool $theme=false):string;

    /**
     * ## password 
     * allow to ask user a password in the terminal 
     * @param string $input input question before interaction
     * @param bool $theme optionnal $theme with default="false" colorful""
     */
    public static function password(string $input,?Bool $theme=false):void;

    /**
     * ## select 
     * allow to add a input in the terminal 
     *  
     * ```php
     *  $select = Dialoguer::select("1x1 = ?",["1","Terrence Howard nonsence"]);
     *  // the terminal will ask you "1x1 = ?" 
     *  // you will have the format list :
     *  // > 1
     *  //   Terrence Howard nonsence 
     *  var_dump($select) // string (1) "1"
     * ```
     * @param string $input input question before interaction
     * @param bool $theme optionnal $theme with default="false" colorful""
     */
    public static function select(string $input,array $list,?Bool $theme=false):string;
    /**
     * ## multiSelect 
     * allow to add a input in the terminal 
     *  
     * ```php
     *  $multiSelect = Dialoguer::multiSelect("?",["1","2","3"]);
     *  // the terminal will ask you "?" 
     *  // you will have the format list :
     *  // [x] 1
     *  // [ ] 2
     *  // [ ] 3
     *  //   Terrence Howard nonsence 
     *  // you add the input "marie"
     *  var_dump($multiSelect) // Array (2) ["1","2"]
     * ```
     * @param string $input input question before interaction
     * @param bool $theme optionnal $theme with default="false" colorful""
     */
    public static function multiSelect(string $input,array $list,?Bool $theme=null):Array;
    /**
     * ## editor 
     * @param string $input input question before interaction
     */
    public static function editor(string $input):void;
}
<?php

$class = $argv[1];

$command = Dialoguer::input("Name your command ?",true);


$option = "";
$param = "";
if(Dialoguer::confirm("Do you want to had options",true)){
    $option .= "\t\tOption([\n";
    $confirm = true;
    $param = "\n";
    while($confirm){
        $option_name = Dialoguer::input("Name your option?",true);
        $input_bool = Dialoguer::confirm("Does that option could have input ? ",true);
        $format = ($input_bool)?"true":"false";
        $option .= "\t\t\t\"-$option_name\"=> new Option_CLI($format,\"test option with input\"),\n";
        $param .= "\t\t" . (($input_bool)?"null|bool|string":"null|bool") . " \$$option_name,\n";
        $confirm = Dialoguer::confirm("Do you want to had a other option ",true);
    }
    $option .= "\t\t]),\n";
}

echo implode("\n",[
    "<?php",
    "namespace " . getenv("NAMESPACE") .';',
    "",
    'use Stampy\Model\Abstract\AbstractPrompsController;',
    'use Stampy\Model\Class\Object\Option_CLI;',
    'use Stampy\Model\Attributes\Description;',
    'use Stampy\Model\Attributes\Command;',
    'use Stampy\Model\Attributes\Option;',
    'use Stampy\Model\Attributes\StdErr;',
    'use Stampy\Model\Attributes\StdOut;',
    'use Stampy\Model\Attributes\StdIn;',
    "",
    "class $class extends AbstractPrompsController",
    "{",
    "\t#[",
    "\t\tCommand('$command'),",
    "$option",
    "\t\tDescription('Test function'),",
    "\t\tStdErr(\"error.log\"),",
    "\t\tStdIn(\"input-file.json\"),",
    "\t\tStdOut(\"output-file.txt\"),",
    "\t]",
    "\tpublic function $command($param){",
    "\t\t/* --- code TODO ---- */",
    "\t}",
    "}"
]);
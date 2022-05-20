<!DOCTYPE html>
<html>
<head>
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Level 2</title>
</head>
<body>
<center>
<h1>File Inclusion - Level 2</h1>

<form action="/file-inclusion/level2.php" method="GET">
<input type="text" name="file" placeholder="Enter file name" />
<input type="submit" value="Submit" />
</form>
</center>

<div style="text-align:center;">

<?php
    
function str_replace_first($search, $replace, $subject)
{
    $search = '/' . preg_quote($search, '/') . '/';
    return preg_replace($search, $replace, $subject, 1);
}

    
if(isset($_GET["file"])){
    $file = $_GET["file"];
    $file = str_replace( array( "http://", "https://"), "", $file );
    $file = str_replace_first("/", "", $file);
    include($file);
}
?>

</body>
</html>

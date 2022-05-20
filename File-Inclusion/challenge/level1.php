<!DOCTYPE html>
<html>
<head>
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Level 1</title>
</head>
<body>
<center>
<h1>File Inclusion - Level 1</h1>

<form action="/file-inclusion/level1.php" method="GET">
<input type="text" name="file" placeholder="Enter file name" />
<input type="submit" value="Submit" />
</form>
</center>

<div style="text-align:center;">

<?php
if(isset($_GET["file"])){
    $file = $_GET["file"];
    if(file_exists($file)){
            echo "<h2>Contents of $file</h2>";
            echo "<pre>";
            echo htmlspecialchars(file_get_contents($file));
            echo "</pre>";
        }
    else{
            $encodedValues = htmlspecialchars($file);
            echo "<h2>The file '$encodedValues' does not exist.</h2>";
        }
}
?>

</body>
</html>

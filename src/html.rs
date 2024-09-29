pub fn ok()->& 'static str {
r##"<!DOCTYPE html>
<html>
<head>
    <title>Ok</title>
    <meta charset="utf-8">
</head>
<body>
    <h1>Request succeeded</h1>
                    
    <p>
        Successfully processed the request.
    </p>
</body>
</html>
"##
} 

pub fn not_found()->& 'static str {
r##"<!DOCTYPE html>
<html>
<head>
    <title>Not Found</title>
    <meta charset="utf-8">
</head>
<body>
    <h1>Not Found</h1>
                    
    <p>
        Sorry, I cannot find what you're looking for.
    </p>
</body>
</html>
"##
} 
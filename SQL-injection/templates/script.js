/*

inspiration: 
https://dribbble.com/shots/2292415-Daily-UI-001-Day-001-Sign-Up

*/

let form = document.querySelector('form');

form.addEventListener('submit', (e) => {
    console.log("running this shit");

    fetch("/login", {
      method: "post",
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
      name: "test",
      password: "test", 
      })
    }).then( (response) => { 
        console.log(response);
    });
});

var jsdom = require('jsdom');
var Promise = require('promise');
var SerialPort = require("serialport").SerialPort;


var serialPort = new SerialPort("/dev/tty.usbserial-AH034JRA", {
  baudrate: 9600
});

serialPort.open(function(error) {
  if (error) {
    console.log("failed to open: "+error);
  } else {

    serialPort.on("data", function(data) {
      console.log("data received: " + data);

      get_next_depature(data).then(function(depature) {
        console.log("depature: ", depature);
        serialPort.write(depature, function(err, results) {
          if (err)
            console.log("write error: " + err);

          console.log("write results: " + results);
        });
      });
    });
  }
});

function get_next_depature(train) {

  var promise = new Promise(function(resolve, reject) {

    jsdom.env(
      "https://reiseplanlegger.ruter.no/no/Sanntid/For/(3012100)Sinsen%20%5BT-bane%5D%20(Oslo)",
      ["http://code.jquery.com/jquery.js"],
      function(errors, window) {
        var $ = window.$;
    
        var trains = $('li').map(parseTrain).filter(function(val) { return val !== undefined; });
    
        function parseTrain() {
    
          var number = $(this).find('span.numBox').text();
          var title = $(this).find('span.title').text();
    
          if (number === '') return;
    
          var depatures = $(this).find('span.item').map(parseDepatures);
          if (number == train) { 
            resolve(depatures[0]);
          }
        }
    
        function parseDepatures() {
          return $(this).text();
        }
    
      }
    );
  });

  return promise;
}

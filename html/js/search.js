window.onload = function()
{
	resTable = document.getElementById("results-table");

	var query = getUrlParam("query");
  var gif = getUrlParam("gif");
	makeAsyncRequest("/cgi-bin/search?query=" + query + "&gif=" + gif, handleJsonResponse);
}

var makeAsyncRequest = function makeAsyncRequest(url, callback)
{
	var req = new XMLHttpRequest();
	req.onload = callback;
	req.responseType = "json";

	req.open("GET", url);
	req.send();
}

var handleJsonResponse = function handleJsonResponse(e)
{
	resTable.innerText = "";
	var jsonResp = e.target.response;
	var table = document.getElementById("results-table");
	var emojiCntr = 0;
	var currRow;
	var currCell;

	for (show in jsonResp)
	{
		for (emoji in jsonResp[show])
		{
			if (emojiCntr % 4 == 0)
			{
				currRow = table.insertRow();
			}
			currCell = currRow.insertCell();
			
			currCell.appendChild(
				generateNewThumbHolder(show, jsonResp[show][emoji]));
			currCell.innerHTML += "<b>" + show + "</b><br />";
			currCell.innerHTML += jsonResp[show][emoji] + "<br />&nbsp;";

			var fullLink = document.createElement("a");

			fullLink.text = "Full";
			fullLink.href = getFullImgUrl(show, jsonResp[show][emoji]);
			currCell.appendChild(fullLink);

			currCell.innerHTML += " | 64x64<br />&nbsp;"

			emojiCntr++;
		}
	}
	if (emojiCntr == 0)
	{
		currCell = table.insertRow().insertCell();
		currCell.innerHTML = "No results found";

	}
}

var generateNewThumbHolder = function generateNewThumbHolder(show, file)
{
	var thumbHolder = document.createElement("div");
	thumbHolder.classList.add("thumbnail-holder");

	var imgUrl = getFullImgUrl(show, file);
	var image = new Image();
	
	image.src = imgUrl;
	image.classList.add("thumbnail");
	
	thumbHolder.appendChild(image);
	
	return thumbHolder;
}

var getFullImgUrl = function getFullImgUrl(show, file)
{
	var emojisRoot = "/emojis/";
	
	if (file.toLowerCase().endsWith(".gif"))
	{
		return emojisRoot + show + "/gifs/" + file;
	}
	else
	{
		return imgUrl = emojisRoot + show + "/" + file;
	}
}

// Code sample, with modifications, comes from 
// https://stackoverflow.com/questions/45758837/script5009-urlsearchparams-is-undefined-in-ie-11
var getUrlParam = function getUrlParam(name){
	var results = 
		new RegExp('[\?&]' + name + '=([^&#]*)').exec(window.location.href);

	if (results == null || !results[1])
	{
		return null;
	} 
	else 
	{
		return decodeURI(results[1]) || 0;
	}
}

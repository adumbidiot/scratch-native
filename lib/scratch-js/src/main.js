export class Game {
	constructor(id){
		this.ctx = document
			.getElementById(id)
			.getContext('2d');
		this.FPS = 60;
		this.loop = null;
		this.children = [];
		
		this.audioAssets = new Map();
		
		this.data = new Map();
		this.data.set("GAME_FIRST_CYCLE", true);
		
		this.chromeButton = null;
		if(window.chrome){
			let div = document.createElement('div');
			div.innerHTML = '<div style="width: 100%; height: 100%; background-color: orange; position: absolute; top: 0px; left: 0px;">' +
				'<div style="width: 50%; height: 10%; background-color: red; font-size: 30px; position: relative; top: 20%; left: 25%; text-align: center;">Press to enable audio</div>' +
				'<div style="position: relative; top: 40%; width: 50%; left: 25%; text-align: center;">You may be wondering what this is all about. Chrome has recently released an update that stops vidoes and audio from playing until the user interacts with the page. For more info, click <a href="https://developers.google.com/web/updates/2017/09/autoplay-policy-changes">here.</a></div>' +
			'</div>';
			this.chromeButton = div;
		}
	}
	
	update(){
		for(let i = 0; i != this.children.length; i++){
			this.children[i].update();
		}
	}
	
	render(){
		this.ctx.clearRect(0, 0, this.ctx.canvas.width, this.ctx.canvas.height);
		for(let i = 0; i != this.children.length; i++){
			this.children[i].render(this.ctx);
		}
	}
	
	add(sprite){
		this.children.push(sprite);
	}
	
	start(){
		if(this.chromeButton){
			document.body.appendChild(this.chromeButton);
			this.chromeButton.onclick = () => {
				this.chromeButton.style.cssText = "display: none;";
				this.chromeButton = null;
				this.innerStart();
			};
		}
	}
	
	innerStart(){
		this.update();
		this.render();
		this.data.set("GAME_FIRST_CYCLE", false);
		this.loop = setInterval(function(){
			this.update();
			this.render();
		}.bind(this), 1000/this.FPS);
	}
	
	stop(){
		clearInterval(this.loop);
	}
	
	addAudioAsset(name, path){
		this.audioAssets.set(name, 'assets/' +path);
	}
}

export class Sprite{
	constructor(){
		this.x = 0;
		this.y = 0;
		this.costumes = [];
		this.activeCostumeIndex = 0;
	}
	
	update(){
		
	}
	
	render(ctx){
		let img = this.costumes[this.activeCostumeIndex].img;
		let x = this.x + (480 - this.costumes[this.activeCostumeIndex].x) / 2;
		let y = (360 - this.costumes[this.activeCostumeIndex].y) / 2 - this.y;
		ctx.drawImage(img, x, y, img.width, img.height);
	}
}

export class Costume{
	constructor(){
		this.img = new Image();
		this.x = 0;
	}
}
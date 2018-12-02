var Scratch = (function (exports) {
	'use strict';

	class Game {
		constructor(id){
			this.ctx = document
				.getElementById(id)
				.getContext('2d');
			this.FPS = 60;
			this.loop = null;
			this.children = [];
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
			this.loop = setInterval(function(){
				this.update();
				this.render();
			}.bind(this), 1000/this.FPS);
		}
		
		stop(){
			cancelInterval(this.loop);
		}
	}

	class Sprite{
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

	class Costume{
		constructor(){
			this.img = new Image();
			this.x = 0;
		}
	}

	exports.Game = Game;
	exports.Sprite = Sprite;
	exports.Costume = Costume;

	return exports;

}({}));

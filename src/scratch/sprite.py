class Sprite:
	def __init__(self, x=0, y=0, costume_index=0):
		self.x = x
		self.y = y
		self.costume_index = costume_index
		self.costumes = []
	def render(self, screen):
		costume = self.costumes[sprite.costume_index]
		render_y = (360 / 2) - self.y - (costume.y / costume.resolution)
		render_x = (480 / 2) + self.x - (costume.x / costume.resolution)
		screen.blit(costume.image, (render_x, render_y))

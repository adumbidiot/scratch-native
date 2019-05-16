class Sprite:
	def __init__(self, x=0, y=0, costume_index=0, direction=90, size=100):
		self.x = x
		self.y = y
		self.costume_index = costume_index
		self.costumes = []
		self.direction = direction
		self.size = size
	def render(self, screen):
		costume = self.costumes[sprite.costume_index]
		render_y = (360 / 2) - self.y - (costume.y / costume.resolution)
		render_x = (480 / 2) + self.x - (costume.x / costume.resolution)
		
		scale = self.size / 100
		scaled_width = scale * costume.get_width()
		scaled_height = scale * costume.get_height()
		scaled_image = costume.get_image(scale)
		
		rot_image = pygame.transform.rotate(scaled_image, 90 - self.direction)
		rot_rect = rot_image.get_rect(center=(render_x, render_y))
		screen.blit(rot_image, rot_rect)

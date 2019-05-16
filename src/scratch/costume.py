class Costume:
	def __init__(self, x=0, y=0, resolution=1, path=""):
		self.x = x
		self.y = y
		self.resolution = resolution
		self.image = self.load_image(path)
	def get_width(self):
		return self.image.get_width()
	def get_height(self):
		return self.image.get_height()
	def load_image(self, path):
		if path.endswith('.svg'):
			svg = Parser.parse_file(path)
			rast = Rasterizer()
			buff = rast.rasterize(svg, svg.width, svg.height)
			image = pygame.image.frombuffer(buff, (svg.width, svg.height), 'RGBA')
			return image
		else:
			image = pygame.image.load(path)
			width = int(image.get_width() / self.resolution)
			height = int(image.get_height() / self.resolution)
			return pygame.transform.scale(image, (width, height))

svg_rasterizer = Rasterizer()
class Costume:
	def __init__(self, x=0, y=0, resolution=1, path=""):
		self.x = x
		self.y = y
		self.resolution = resolution
		self.svg = None
		self.image = None
		self.load_image(path)
	def get_width(self):
		if self.svg != None:
			return self.svg.width
		else:
			return self.image.get_width()
	def get_height(self):
		if self.svg != None:
			return self.svg.height
		else:
			return self.image.get_height()
	def get_image(self, scale=1):
		if scale == 0:
			scale = 1 #TODO: Empty image
		if self.svg != None:
			scaled_width = int(self.svg.width * scale)
			scaled_height = int(self.svg.height * scale)
			buff = svg_rasterizer.rasterize(self.svg, scaled_width, scaled_height, scale)
			return pygame.image.frombuffer(buff, (scaled_width, scaled_height), 'RGBA')
		else:
			return pygame.transform.scale(self.image, (w, h))
	def load_image(self, path):
		if path.endswith('.svg'):
			self.svg = Parser.parse_file(path)
		else:
			image = pygame.image.load(path)
			width = int(image.get_width() / self.resolution)
			height = int(image.get_height() / self.resolution)
			self.image = pygame.transform.scale(image, (width, height))

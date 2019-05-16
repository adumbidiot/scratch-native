class EventDispatcher:
	def __init__(self): 
		self.handlers = {}
		self.unfinished = []
	
	def on(self, type, handler):
		if type not in self.handlers:
			self.handlers[type] = [];
		self.handlers[type].append(handler)
	def fire(self, event):
		if event.type in self.handlers:
			for handler in self.handlers[event.type]:
				func = handler(event)
				if func != None:
					try:
						next(func)
						self.unfinished.append(func)
					except StopIteration:
						pass
				
	def update(self):
		for func in self.unfinished:
			try:
				next(func)
			except StopIteration:
				self.unfinished.remove(func)
				pass

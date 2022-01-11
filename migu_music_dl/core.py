class Singer:

    def __init__(self, id, name):
        self.id = id
        self.name = name

    def __str__(self):
        return self.id + ' ' + self.name


class Album:

    def __init__(self, id, name):
        self.id = id
        self.name = name

    def __str__(self):
        return self.id + ' ' + self.name


class Song:

    def __init__(self, id, name, image_url, download_url, singers, albums):
        self.id = id
        self.name = name
        self.image_url = image_url
        self.download_url = download_url
        self.singers = singers
        self.albums = albums

    def __str__(self):
        return '{' + self.name + '\n' + self.download_url + '\n' + self.image_url + '\n' + str(
            self.singers) + '\n' + str(self.albums) + '}'

from migu_music_dl.extenral_api import MiguMusicApi


def test_search():
    api = MiguMusicApi()
    result = api.search('周杰伦')
    for song in result:
        print(song)
    assert len(result) > 0

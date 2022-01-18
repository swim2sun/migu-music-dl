import requests
import logging
from .core import Singer, Album, Song

logger = logging.getLogger(__name__)


class MiguMusicApi:

    def __init__(self):
        self.session = requests.Session()

    def search(self, key_word: str, page_number: int = 1, page_size: int = 10, quality: str = 'SQ'):
        params = {
            'ua': 'Android_migu',
            'version': '5.0.1',
            'pageNo': page_number,
            'pageSize': page_size,
            'text': key_word,
            'searchSwitch': '{"song":1,"album":0,"singer":0,"tagSong":0,"mvSong":0,"songlist":0,"bestShow":1}'
        }
        url = 'http://pd.musicapp.migu.cn/MIGUM2.0/v1.0/content/search_all.do'
        resp = self.session.get(url, params=params, timeout=8)
        if resp.status_code != requests.codes.ok:
            raise RuntimeError(f'{url} response with status code: {resp.status_code}')
        if not resp.text:
            raise RuntimeError(f'{url} response with empty text')
        resp_json = resp.json()
        total_count = resp_json['songResultData']['totalCount']
        logger.info('search total count: %d', total_count)
        songs = []
        for song_info in resp_json['songResultData']['result']:
            singers = [Singer(name=s['name'], id=s['id']) for s in song_info['singers']]
            albums = [Album(name=a['name'], id=a['id']) for a in song_info['albums']] if 'albums' in song_info else []
            image_items = song_info['imgItems']
            image_url = image_items[0]['img'] if len(image_items) > 0 else None
            tone_type = 'SQ&formatType=SQ&resourceType=E'
            if quality == 'HQ':
                tone_type = 'HQ&formatType=HQ&resourceType=2'
            download_url = f'http://218.205.239.34/MIGUM2.0/v1.0/content/sub/listenSong.do?toneFlag={tone_type}' \
                           f'&netType=00&copyrightId=0&&contentId={song_info["contentId"]}&channel=0'
            song = Song(id=song_info['id'],
                        name=song_info['name'],
                        image_url=image_url,
                        download_url=download_url,
                        singers=singers,
                        albums=albums)
            songs.append(song)
        return songs, total_count

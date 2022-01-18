import os.path
from mimetypes import guess_extension

import click
import requests
from prettytable import PrettyTable

from .extenral_api import MiguMusicApi


@click.command()
@click.version_option()
@click.option('-s', '--quality', default='SQ', help='Quality of the music',
              type=click.Choice(['SQ', 'HQ'], case_sensitive=False))
@click.option('-ps', '--page-size', default='20', help='Page size')
@click.argument('keyword')
@click.argument('output_dir')
def download(quality, keyword, output_dir, page_size):
    api = MiguMusicApi()
    page_number = 1
    selected_songs = []
    while len(selected_songs) == 0:
        songs, total_count = api.search(keyword, page_number=page_number, quality=quality)
        if len(songs) == 0:
            click.echo(click.style('No songs found', fg='red'))
            return
        show_songs(songs)
        next_page = click.confirm('next page?', default=True)
        if next_page:
            page_number += 1
            click.clear()
            continue
        selected_songs = wait_select_songs(songs)
        break
    if len(selected_songs) == 0:
        click.echo(click.style('No songs selected', fg='red'))
        return
    for song in selected_songs:
        download_file(song.name, song.download_url, output_dir)


def show_songs(songs):
    table = PrettyTable(['No.', 'Title', 'Artist', 'Album'])
    table.align['Title'] = 'l'
    for i in range(len(songs)):
        song = songs[i]
        table.add_row(
            [click.style(i + 1, fg='green'),
             click.style(song.name, fg='blue', bold=True),
             ', '.join([singer.name for singer in song.singers]),
             click.style(', '.join([album.name for album in song.albums]), bold=True)]
        )
    click.echo(table)


def wait_select_songs(songs):
    click.echo(f'input {click.style("No.", fg="green")} to download '
               f'(split with {click.style(",", fg="green", bold=True)} for download multiple songs, '
               f'for example: 1,3,5): ',
               nl=False)
    selected = click.get_text_stream('stdin').readline().strip()
    selected_songs = []
    for index in selected.split(',') if selected else []:
        try:
            selected_songs.append(songs[int(index) - 1])
        except IndexError:
            click.echo(click.style('Invalid index', fg='red'))
    click.echo(f'selected: {", ".join([song.name for song in selected_songs])}')
    return selected_songs


def download_file(name, url, output_dir):
    response = requests.get(url, stream=True)
    total_length = response.headers.get('content-length')
    content_type = response.headers.get('content-type')
    extension = guess_extension(content_type.partition(';')[0].strip())
    file_name = name + extension
    with open(os.path.join(output_dir, file_name), "wb") as f:
        dl = 0
        total_length = int(total_length)
        with click.progressbar(length=total_length, label=file_name) as bar:
            for data in response.iter_content(chunk_size=4096):
                dl += len(data)
                f.write(data)
                bar.update(dl)


if __name__ == '__main__':
    download()

let f = new FormData();
f.append('q', 'https://www.youtube.com/watch?v=JhRDFLVRfmg');
f.append('vt', 'home');
let r = await(await fetch('https://yt1s.com/api/ajaxSearch/index', {
    method: 'POST',
    // headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
    body: f,
})).json()

console.log(r.links.mp4["auto"].k)

let g = new FormData();
g.append('vid', r.vid);
g.append('k',/* encodeURIComponent */(r.links.mp4["auto"].k));
let s = await(await fetch('https://yt1s.com/api/ajaxConvert/convert', {
    method: 'POST',
    // headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
    body: g,
})).json()

console.log(s.dlink);

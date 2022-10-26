import { useNavigate, useParams } from "react-router-dom";
import AlbumDetails from "../../Components/AlbumDetails";
import { useGetAlbumQuery } from "../../Hooks/GraphQL";
import { useTimeFormat } from "../../Hooks/useFormat";
import { usePlayback } from "../../Hooks/usePlayback";

const AlbumDetailsPage = () => {
  const params = useParams();
  const { data, loading } = useGetAlbumQuery({
    variables: {
      id: params.id!,
    },
  });
  const navigate = useNavigate();
  const { formatTime } = useTimeFormat();
  const { play, pause, next, previous, nowPlaying } = usePlayback();
  const album =
    !loading && data
      ? {
          ...data.album,
          tracks: data.album.tracks.map((track) => ({
            "#": track.trackNumber,
            id: track.id,
            title: track.title,
            artist: track.artists.map((artist) => artist.name).join(", "),
            time: formatTime(track.duration! * 1000),
          })),
        }
      : { tracks: [] };
  return (
    <AlbumDetails
      onBack={() => navigate(-1)}
      onClickLibraryItem={(item) => navigate(`/${item}`)}
      onClickAlbum={() => {}}
      onPlay={() => play()}
      onPause={() => pause()}
      onNext={() => next()}
      onPrevious={() => previous()}
      onShuffle={() => {}}
      onRepeat={() => {}}
      album={album}
      nowPlaying={nowPlaying}
    />
  );
};

export default AlbumDetailsPage;

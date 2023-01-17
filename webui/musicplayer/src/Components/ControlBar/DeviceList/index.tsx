import { css, useTheme } from "@emotion/react";
import styled from "@emotion/styled";
import { ListItem, ListItemLabel } from "baseui/list";
import { FC } from "react";
import { Device } from "../../../Types/Device";
import { MusicPlayer } from "@styled-icons/bootstrap";
import { Laptop } from "@styled-icons/ionicons-outline";
import { Kodi, Airplayaudio, Chromecast } from "@styled-icons/simple-icons";

const Container = styled.div`
  max-height: calc(100vh - 153px); /* - 90px */
  padding-top: 15px;
  padding-bottom: 15px;
  overflow-y: auto;
`;

const List = styled.div`
  max-height: calc(100vh - 273px); /* - 210px */
  padding-left: 15px;
  padding-right: 15px;
  overflow-y: auto;
`;

const Icon = styled.div`
  height: 40px;
  width: 40px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: ${(props) => props.theme.colors.cover};
  ${(props) =>
    props.color &&
    css`
      background-color: ${props.color};
    `}
`;

const Title = styled.div`
  margin: 10px;
  margin-left: 25px;
  margin-right: 25px;
  font-family: "RockfordSansBold";
`;

const CurrentDeviceWrapper = styled.div`
  height: 60px;
  display: flex;
  margin-left: 25px;
  margin-right: 25px;
  align-items: center;
`;

const CurrentDevice = styled.div`
  font-size: 18px;
`;

const CurrentDeviceName = styled.div`
  color: #ab28fc;
  font-size: 14px;
`;

const IconWrapper = styled.div`
  margin-top: 3px;
  margin-right: 16px;
`;

export type ArtworkProps = {
  icon?: string;
  color?: string;
};

const Artwork: FC<ArtworkProps> = ({ icon, color }) => {
  const theme = useTheme();
  return (
    <Icon color={color}>
      {icon === "music-player" && <MusicPlayer size={18} color="#28fce3" />}
      {icon === "xbmc" && <Kodi size={18} color="#28cbfc" />}
      {icon === "airplay" && <Airplayaudio size={18} color={"#ff00c3"} />}
      {icon === "chromecast" && (
        <Chromecast size={18} color={theme.colors.text} />
      )}
    </Icon>
  );
};

Artwork.defaultProps = {
  icon: "music-player",
};

const DeviceName = styled.div`
  font-size: 14px;
  color: "#ab28fc";
`;

export type DeviceListProps = {
  castDevices: Device[];
};

const DeviceList: FC<DeviceListProps> = ({ castDevices }) => {
  const theme = useTheme();
  const colors: {
    [key: string]: string;
  } = {
    "music-player": "rgba(40, 252, 227, 0.088)",
    xbmc: "rgba(40, 203, 252, 0.082)",
    airplay: "rgba(255, 0, 195, 0.063)",
  };
  return (
    <Container>
      <CurrentDeviceWrapper>
        <IconWrapper>
          <Laptop size={30} color={"#ab28fc"} />
        </IconWrapper>
        <div>
          <CurrentDevice>Current device</CurrentDevice>
          <CurrentDeviceName>Music Player</CurrentDeviceName>
        </div>
      </CurrentDeviceWrapper>
      <Title>Select another output device</Title>
      <List>
        {castDevices.map((device) => (
          <ListItem
            key={device.id}
            artwork={() => (
              <Artwork icon={device.type} color={colors[device.type]} />
            )}
            overrides={{
              Root: {
                style: {
                  cursor: "pointer",
                  ":hover": {
                    backgroundColor: theme.colors.hover,
                  },
                  borderRadius: "5px",
                },
              },
              Content: {
                style: {
                  borderBottom: "none",
                },
              },
            }}
          >
            <ListItemLabel>
              <DeviceName>{device.name}</DeviceName>
            </ListItemLabel>
          </ListItem>
        ))}
      </List>
    </Container>
  );
};

DeviceList.defaultProps = {};

export default DeviceList;

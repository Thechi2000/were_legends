export interface Data {
  roles: {
    [key: string]: {
      name: string;
      description: string;
    };
  };
  missions: {
    [key: string]: string;
  };
}
